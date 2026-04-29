import { startLogin } from '../../lib/auth/keycloak'

export function LoginPage() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="max-w-md w-full p-8 bg-white rounded-lg shadow-md">
        <h1 className="text-2xl font-bold text-center mb-8">
          Schreinerei App
        </h1>
        <button
          onClick={startLogin}
          className="w-full py-3 px-4 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors"
        >
          Mit Keycloak anmelden
        </button>
      </div>
    </div>
  )
}
