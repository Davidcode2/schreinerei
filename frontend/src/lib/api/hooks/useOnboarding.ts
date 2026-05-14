import { useMutation, useQuery } from "@tanstack/react-query"
import { apiClient } from "../client"
import type {
  CreateOnboardingSessionRequest,
  OnboardingSessionResponse,
  OnboardingSessionStatusResponse,
} from "@/types/generated"

const ACTIVE_STATUSES = new Set([
  "pending_payment",
  "payment_confirmed",
  "provisioning",
])

export function useCreateOnboardingSession() {
  return useMutation({
    mutationFn: (data: CreateOnboardingSessionRequest) =>
      apiClient.post<OnboardingSessionResponse>("/api/v1/onboarding/sessions", data),
  })
}

export function useOnboardingSession(sessionId: string | null) {
  return useQuery({
    queryKey: ["onboarding-session", sessionId],
    queryFn: () =>
      apiClient.get<OnboardingSessionStatusResponse>(
        `/api/v1/onboarding/sessions/${encodeURIComponent(sessionId ?? "")}`
      ),
    enabled: Boolean(sessionId),
    retry: false,
    refetchInterval: (query) => {
      const status = query.state.data?.status
      return status && ACTIVE_STATUSES.has(status) ? 2000 : false
    },
  })
}
