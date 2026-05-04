import { describe, expect, it, vi, beforeEach } from "vitest"
import userEvent from "@testing-library/user-event"
import { render, screen, waitFor } from "@/test/utils"
import { CameraUploadFlow } from "./CameraUploadFlow"

const createActivityMutate = vi.fn()
const uploadPhotoMutate = vi.fn()
const queuePhotoUploadActionFn = vi.fn()

vi.mock("@/lib/api/hooks", () => ({
  useCreateActivity: () => ({ isPending: false, mutateAsync: createActivityMutate }),
  useUploadSitePhoto: () => ({ isPending: false, mutateAsync: uploadPhotoMutate }),
}))

vi.mock("@/lib/offline/sync", () => ({
  isOnline: vi.fn(),
}))

vi.mock("@/lib/offline/queue", () => ({
  queuePhotoUploadAction: (...args: unknown[]) => queuePhotoUploadActionFn(...args),
}))

import { isOnline } from "@/lib/offline/sync"

async function selectFile(container: Document) {
  const fileInput = container.querySelector('input[type="file"]') as HTMLInputElement
  const file = new File(["photo-data"], "test.jpg", { type: "image/jpeg" })
  await userEvent.upload(fileInput, file)
  return file
}

describe("CameraUploadFlow", () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it("renders camera file input with capture=environment and accept=image/*", () => {
    render(
      <CameraUploadFlow
        open
        onOpenChange={vi.fn()}
        siteId="site-1"
        onSuccess={vi.fn()}
      />
    )

    const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement
    expect(fileInput).toBeInTheDocument()
    expect(fileInput).toHaveAttribute("accept", "image/*")
    expect(fileInput).toHaveAttribute("capture", "environment")
  })

  it("shows image preview after file selection", async () => {
    render(
      <CameraUploadFlow
        open
        onOpenChange={vi.fn()}
        siteId="site-1"
        onSuccess={vi.fn()}
      />
    )

    await selectFile(document)

    expect(screen.getByAltText("Vorschau")).toBeInTheDocument()
    expect(screen.getByText("Hochladen")).toBeInTheDocument()
  })

  it("shows optional note textarea after file selection", async () => {
    render(
      <CameraUploadFlow
        open
        onOpenChange={vi.fn()}
        siteId="site-1"
        onSuccess={vi.fn()}
      />
    )

    await selectFile(document)

    expect(screen.getByPlaceholderText("Notiz hinzufügen (optional)...")).toBeInTheDocument()
  })

  it("rejects photos larger than 10 MB before upload", async () => {
    vi.mocked(isOnline).mockReturnValue(true)

    render(
      <CameraUploadFlow
        open
        onOpenChange={vi.fn()}
        siteId="site-1"
        onSuccess={vi.fn()}
      />
    )

    const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement
    const oversizedFile = new File([new Uint8Array(10 * 1024 * 1024 + 1)], "large.jpg", {
      type: "image/jpeg",
    })
    await userEvent.upload(fileInput, oversizedFile)

    expect(screen.queryByAltText("Vorschau")).not.toBeInTheDocument()
    expect(uploadPhotoMutate).not.toHaveBeenCalled()
    expect(createActivityMutate).not.toHaveBeenCalled()
  })

  it("calls upload then createActivity when submitting online", async () => {
    vi.mocked(isOnline).mockReturnValue(true)
    uploadPhotoMutate.mockResolvedValue({ photo_url: "https://example.com/photo.jpg" })
    createActivityMutate.mockResolvedValue({})

    const onSuccess = vi.fn()
    const onOpenChange = vi.fn()
    render(
      <CameraUploadFlow
        open
        onOpenChange={onOpenChange}
        siteId="site-1"
        onSuccess={onSuccess}
      />
    )

    await selectFile(document)
    await userEvent.click(screen.getByRole("button", { name: "Hochladen" }))

    await waitFor(() => {
      expect(uploadPhotoMutate).toHaveBeenCalledWith({
        siteId: "site-1",
        file: expect.any(File),
      })
    })

    await waitFor(() => {
      expect(createActivityMutate).toHaveBeenCalledWith({
        siteId: "site-1",
        activity_type: "photo",
        content: undefined,
        photo_url: "https://example.com/photo.jpg",
      })
    })

    expect(onSuccess).toHaveBeenCalledOnce()
    expect(onOpenChange).toHaveBeenCalledWith(false)
  })

  it("calls queuePhotoUploadAction when submitting offline", async () => {
    vi.mocked(isOnline).mockReturnValue(false)
    queuePhotoUploadActionFn.mockResolvedValue(1)

    const onSuccess = vi.fn()
    const onOpenChange = vi.fn()
    render(
      <CameraUploadFlow
        open
        onOpenChange={onOpenChange}
        siteId="site-1"
        onSuccess={onSuccess}
      />
    )

    const file = await selectFile(document)
    await userEvent.click(screen.getByRole("button", { name: "Hochladen" }))

    await waitFor(() => {
      expect(queuePhotoUploadActionFn).toHaveBeenCalledWith({
        siteId: "site-1",
        file,
        content: undefined,
      })
    })

    expect(uploadPhotoMutate).not.toHaveBeenCalled()
    expect(createActivityMutate).not.toHaveBeenCalled()
    expect(onSuccess).toHaveBeenCalledOnce()
    expect(onOpenChange).toHaveBeenCalledWith(false)
  })

  it("clears form and calls onOpenChange(false) on successful submit", async () => {
    vi.mocked(isOnline).mockReturnValue(true)
    uploadPhotoMutate.mockResolvedValue({ photo_url: "https://example.com/photo.jpg" })
    createActivityMutate.mockResolvedValue({})

    const onSuccess = vi.fn()
    const onOpenChange = vi.fn()
    render(
      <CameraUploadFlow
        open
        onOpenChange={onOpenChange}
        siteId="site-1"
        onSuccess={onSuccess}
      />
    )

    await selectFile(document)
    await userEvent.click(screen.getByRole("button", { name: "Hochladen" }))

    await waitFor(() => {
      expect(onSuccess).toHaveBeenCalledOnce()
      expect(onOpenChange).toHaveBeenCalledWith(false)
    })
  })

  it("shows validation error if submitting without a file selected", async () => {
    vi.mocked(isOnline).mockReturnValue(true)

    const { unmount } = render(
      <CameraUploadFlow
        open
        onOpenChange={vi.fn()}
        siteId="site-1"
        onSuccess={vi.fn()}
      />
    )

    expect(screen.queryByRole("button", { name: "Hochladen" })).not.toBeInTheDocument()

    unmount()
  })

  it("note field is optional — submit works with photo only (no note text)", async () => {
    vi.mocked(isOnline).mockReturnValue(true)
    uploadPhotoMutate.mockResolvedValue({ photo_url: "https://example.com/photo.jpg" })
    createActivityMutate.mockResolvedValue({})

    const onSuccess = vi.fn()
    render(
      <CameraUploadFlow
        open
        onOpenChange={vi.fn()}
        siteId="site-1"
        onSuccess={onSuccess}
      />
    )

    await selectFile(document)
    await userEvent.click(screen.getByRole("button", { name: "Hochladen" }))

    await waitFor(() => {
      expect(createActivityMutate).toHaveBeenCalledWith({
        siteId: "site-1",
        activity_type: "photo",
        photo_url: "https://example.com/photo.jpg",
      })
    })

    expect(onSuccess).toHaveBeenCalledOnce()
  })
})
