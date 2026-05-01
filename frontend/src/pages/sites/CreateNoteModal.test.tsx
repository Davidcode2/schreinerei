import { beforeEach, describe, expect, it, vi } from "vitest"
import userEvent from "@testing-library/user-event"
import { render, screen } from "@/test/utils"
import { CreateNoteModal } from "./CreateNoteModal"

const createActivityMutate = vi.fn()
const uploadSiteAttachmentMutate = vi.fn()

vi.mock("@/lib/api/hooks", () => ({
  useCreateActivity: () => ({ isPending: false, mutateAsync: createActivityMutate }),
  useUploadSitePhoto: () => ({ isPending: false, mutateAsync: vi.fn() }),
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

  it("allows removing a selected file before submit", async () => {
    renderModal()
    const user = userEvent.setup()

    const picker = document.querySelector('input[type="file"]') as HTMLInputElement
    const pdfFile = new File(["pdf"], "angebot.pdf", { type: "application/pdf" })
    await user.upload(picker, pdfFile)

    await user.click(screen.getByRole("button", { name: "Datei entfernen: angebot.pdf" }))

    expect(screen.queryByText("angebot.pdf")).not.toBeInTheDocument()
  })
})
