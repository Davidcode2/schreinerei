import { describe, it, expect, vi, afterEach } from "vitest"
import { renderHook } from "@testing-library/react"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { useCreateActivity, useDeleteActivity, useUploadSiteAttachment, useUploadSitePhoto } from "./useSites"
import type { Activity } from "@/types/sites"
import { apiClient } from "../client"

vi.mock("../client", () => ({
  apiClient: {
    post: vi.fn(),
    delete: vi.fn(),
  },
}))

function createWrapper(queryClient: QueryClient) {
  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
}

function createQueryClient() {
  return new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false },
    },
  })
}

describe("useUploadSitePhoto", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("sends multipart payload and returns attachment metadata", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({
      attachment_id: "att-1",
      photo_url: "/api/v1/sites/site-1/attachments/att-1",
      thumbnail_url: "/api/v1/sites/site-1/attachments/att-1/thumbnail",
    })

    const queryClient = createQueryClient()
    const { result } = renderHook(() => useUploadSitePhoto(), {
      wrapper: createWrapper(queryClient),
    })

    const file = new File(["image"], "photo.jpg", { type: "image/jpeg" })
    const response = await result.current.mutateAsync({ siteId: "site-1", file })

    expect(apiClient.post).toHaveBeenCalledWith(
      "/api/v1/sites/site-1/attachments/photo",
      expect.any(FormData)
    )
    const firstCall = vi.mocked(apiClient.post).mock.calls[0]
    expect(firstCall).toBeDefined()
    const formData = firstCall?.[1] as FormData
    expect(formData.get("photo")).toBe(file)
    expect(formData.get("file")).toBeNull()
    expect(response).toEqual({
      attachment_id: "att-1",
      photo_url: "/api/v1/sites/site-1/attachments/att-1",
      thumbnail_url: "/api/v1/sites/site-1/attachments/att-1/thumbnail",
    })
  })

  it("exposes upload errors for UI handling", async () => {
    vi.mocked(apiClient.post).mockRejectedValueOnce(new Error("Upload failed"))
    const queryClient = createQueryClient()
    const { result } = renderHook(() => useUploadSitePhoto(), {
      wrapper: createWrapper(queryClient),
    })

    const file = new File(["image"], "photo.jpg", { type: "image/jpeg" })

    await expect(
      result.current.mutateAsync({ siteId: "site-1", file })
    ).rejects.toThrow("Upload failed")
  })
})

describe("useCreateActivity", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("uses returned photo_url directly when creating photo activity", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({ id: "activity-1" })
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useCreateActivity(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      siteId: "site-1",
      activity_type: "photo",
      photo_url: "/api/v1/sites/site-1/attachments/att-1",
    })

    expect(apiClient.post).toHaveBeenCalledWith("/api/v1/sites/site-1/activities", {
      activity_type: "photo",
      photo_url: "/api/v1/sites/site-1/attachments/att-1",
    })
  })

  it("sends attachment ids when creating a document activity", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({ id: "activity-2" })
    const queryClient = createQueryClient()

    const { result } = renderHook(() => useCreateActivity(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({
      siteId: "site-1",
      activity_type: "note",
      content: "Montage abgeschlossen",
      attachment_ids: ["att-1", "att-2"],
    })

    expect(apiClient.post).toHaveBeenCalledWith("/api/v1/sites/site-1/activities", {
      activity_type: "note",
      content: "Montage abgeschlossen",
      attachment_ids: ["att-1", "att-2"],
    })
  })
})

describe("useDeleteActivity", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("trusts the typed can_delete contract from the backend", () => {
    const activity: Activity = {
      id: "activity-1",
      site_id: "site-1",
      user_id: "user-1",
      creator_name: "Anna Tischler",
      can_delete: true,
      activity_type: "note",
      content: "Montage abgeschlossen",
      photo_url: null,
      attachments: [],
      created_at: "2026-05-01T10:00:00.000Z",
    }

    expect(activity.can_delete).toBe(true)
  })

  it("issues the site activity delete request and invalidates that feed", async () => {
    vi.mocked(apiClient.delete).mockResolvedValueOnce({ success: true })
    const queryClient = createQueryClient()
    const invalidateQueriesSpy = vi.spyOn(queryClient, "invalidateQueries")

    const { result } = renderHook(() => useDeleteActivity(), {
      wrapper: createWrapper(queryClient),
    })

    await result.current.mutateAsync({ siteId: "site-1", activityId: "activity-1" })

    expect(apiClient.delete).toHaveBeenCalledWith(
      "/api/v1/sites/site-1/activities/activity-1"
    )
    expect(invalidateQueriesSpy).toHaveBeenCalledWith({
      queryKey: ["activities", "site-1"],
    })
  })

  it("surfaces delete failures for UI handling", async () => {
    vi.mocked(apiClient.delete).mockRejectedValueOnce(new Error("Delete failed"))
    const queryClient = createQueryClient()
    const { result } = renderHook(() => useDeleteActivity(), {
      wrapper: createWrapper(queryClient),
    })

    await expect(
      result.current.mutateAsync({ siteId: "site-1", activityId: "activity-1" })
    ).rejects.toThrow("Delete failed")
  })
})

describe("useUploadSiteAttachment", () => {
  afterEach(() => {
    vi.clearAllMocks()
  })

  it("posts files under the generic attachment field", async () => {
    vi.mocked(apiClient.post).mockResolvedValueOnce({
      attachment_id: "att-9",
      filename: "angebot.pdf",
      mime_type: "application/pdf",
      url: "/api/v1/attachments/att-9",
      thumbnail_url: null,
    })

    const queryClient = createQueryClient()
    const { result } = renderHook(() => useUploadSiteAttachment(), {
      wrapper: createWrapper(queryClient),
    })

    const file = new File(["pdf"], "angebot.pdf", { type: "application/pdf" })
    await result.current.mutateAsync({ siteId: "site-1", file })

    expect(apiClient.post).toHaveBeenCalledWith(
      "/api/v1/sites/site-1/attachments",
      expect.any(FormData)
    )

    const firstCall = vi.mocked(apiClient.post).mock.calls[0]
    expect(firstCall).toBeDefined()
    const formData = firstCall?.[1] as FormData
    expect(formData.get("attachment")).toBe(file)
    expect(formData.get("photo")).toBeNull()
  })
})
