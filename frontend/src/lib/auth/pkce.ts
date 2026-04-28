/**
 * PKCE (Proof Key for Code Exchange) utilities for OAuth2
 * 
 * PKCE provides protection against authorization code interception attacks
 * by requiring the client to prove possession of the code_verifier.
 */

/**
 * Generate cryptographically secure random string
 */
export function generateRandomString(length: number): string {
  const array = new Uint8Array(length)
  crypto.getRandomValues(array)
  return Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('')
}

/**
 * Generate code_verifier (43-128 chars)
 * Uses 32 random bytes encoded as base64url (43 chars)
 */
export function generateCodeVerifier(): string {
  const array = new Uint8Array(32)
  crypto.getRandomValues(array)
  return base64UrlEncode(array)
}

/**
 * Generate code_challenge from verifier (S256 method)
 * SHA-256 hash of the verifier, base64url encoded
 */
export async function generateCodeChallenge(verifier: string): Promise<string> {
  const encoder = new TextEncoder()
  const data = encoder.encode(verifier)
  const digest = await crypto.subtle.digest('SHA-256', data)
  return base64UrlEncode(new Uint8Array(digest))
}

/**
 * Base64 URL encode without padding
 * Replaces + with -, / with _, and removes trailing =
 */
function base64UrlEncode(array: Uint8Array): string {
  let binary = ''
  for (let i = 0; i < array.length; i++) {
    binary += String.fromCharCode(array[i])
  }
  return btoa(binary)
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=+$/, '')
}
