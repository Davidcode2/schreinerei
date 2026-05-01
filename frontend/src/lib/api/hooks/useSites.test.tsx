import { describe, it, expect, vi, afterEach } from "vitest"
import { renderHook } from "@testing-library/react"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { useCreateActivity, useUploadSitePhoto } from "./useSites"
import { apiClient } from "../client"

vi.mock("../client", () => ({
  apiClient: {
    post: vi.fn(),
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
})
