import { beforeEach, describe, expect, it, vi } from "vitest"
import userEvent from "@testing-library/user-event"
import { render, screen } from "@/test/utils"
import { CreateNoteModal } from "./CreateNoteModal"

const createActivityMutate = vi.fn()
const uploadPhotoMutate = vi.fn()
const uploadSiteAttachmentMutate = vi.fn()

vi.mock("@/lib/api/hooks", () => ({
  useCreateActivity: () => ({ isPending: false, mutateAsync: createActivityMutate }),
  useUploadSitePhoto: () => ({ isPending: false, mutateAsync: uploadPhotoMutate }),
  useUploadSiteAttachment: () => ({ isPending: false, mutateAsync: uploadSiteAttachmentMutate }),
}))

vi.mock("@/lib/offline/sync", () => ({
  isOnline: vi.fn(() => true),
}))

vi.mock("@/lib/offline/queue", () => ({
  queuePhotoUploadAction: vi.fn(),
}))

describe("CreateNoteModal", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  function renderModal() {
    render(
      <CreateNoteModal
        open
        onOpenChange={vi.fn()}
        siteId="site-1"
        onSuccess={vi.fn()}
      />
    )
  }

  it("renders the document composer copy instead of the old note/photo toggle", () => {
    renderModal()

    expect(screen.queryByRole("button", { name: "Notiz" })).not.toBeInTheDocument()
    expect(screen.queryByRole("button", { name: "Foto" })).not.toBeInTheDocument()
    expect(screen.getByText("Dokumente hinzufügen")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "Dateien auswählen" })).toBeInTheDocument()
    expect(screen.getByText("Unterstützt Bilder und PDFs.")).toBeInTheDocument()
    expect(screen.getByRole("button", { name: "Eintrag speichern" })).toBeDisabled()
  })

  it("keeps valid image and pdf files while rejecting invalid selections", async () => {
    renderModal()
    const user = userEvent.setup({ applyAccept: false })

    const picker = document.querySelector('input[type="file"]') as HTMLInputElement
    const imageFile = new File(["image"], "planung.jpg", { type: "image/jpeg" })
    const pdfFile = new File(["pdf"], "angebot.pdf", { type: "application/pdf" })
    const invalidFile = new File(["text"], "readme.txt", { type: "text/plain" })

    await user.upload(picker, [imageFile, pdfFile, invalidFile])

    expect(screen.getByText("planung.jpg")).toBeInTheDocument()
    expect(screen.getByText("angebot.pdf")).toBeInTheDocument()
    expect(screen.getByText(/Nicht unterstützte Dateien:/i)).toBeInTheDocument()
  })

  it("rejects files larger than 10 MB before upload", async () => {
    renderModal()
    const user = userEvent.setup({ applyAccept: false })

    const picker = document.querySelector('input[type="file"]') as HTMLInputElement
    const oversizedPdf = new File([new Uint8Array(10 * 1024 * 1024 + 1)], "gross.pdf", {
      type: "application/pdf",
    })

    await user.upload(picker, oversizedPdf)

    expect(screen.getByText(/Zu groß \(max\. 10 MB\): gross\.pdf/i)).toBeInTheDocument()
    expect(screen.queryByText("gross.pdf")).not.toBeInTheDocument()
  })

  it("allows removing a selected file before submit", async () => {
    renderModal()
    const user = userEvent.setup()

    const picker = document.querySelector('input[type="file"]') as HTMLInputElement
    const pdfFile = new File(["pdf"], "angebot.pdf", { type: "application/pdf" })
    await user.upload(picker, pdfFile)

    await user.click(screen.getByRole("button", { name: "Datei entfernen: angebot.pdf" }))

    expect(screen.queryByText("angebot.pdf")).not.toBeInTheDocument()
  })

  it("uploads attachments first and then creates one activity payload", async () => {
    renderModal()
    const user = userEvent.setup()

    uploadSiteAttachmentMutate
      .mockResolvedValueOnce({
        attachment_id: "att-1",
        filename: "planung.jpg",
        mime_type: "image/jpeg",
        url: "/api/v1/attachments/att-1",
        thumbnail_url: "/api/v1/attachments/att-1/thumbnail",
      })
      .mockResolvedValueOnce({
        attachment_id: "att-2",
        filename: "angebot.pdf",
        mime_type: "application/pdf",
        url: "/api/v1/attachments/att-2",
        thumbnail_url: null,
      })

    const picker = document.querySelector('input[type="file"]') as HTMLInputElement
    const imageFile = new File(["image"], "planung.jpg", { type: "image/jpeg" })
    const pdfFile = new File(["pdf"], "angebot.pdf", { type: "application/pdf" })

    await user.type(screen.getByPlaceholderText("Notiz hinzufügen (optional)..."), "Montage abgeschlossen")
    await user.upload(picker, [imageFile, pdfFile])
    await user.click(screen.getByRole("button", { name: "Eintrag speichern" }))

    expect(uploadSiteAttachmentMutate).toHaveBeenNthCalledWith(1, {
      siteId: "site-1",
      file: imageFile,
    })
    expect(uploadSiteAttachmentMutate).toHaveBeenNthCalledWith(2, {
      siteId: "site-1",
      file: pdfFile,
    })
    expect(createActivityMutate).toHaveBeenCalledWith({
      siteId: "site-1",
      activity_type: "note",
      content: "Montage abgeschlossen",
      attachment_ids: ["att-1", "att-2"],
    })
  })
})
