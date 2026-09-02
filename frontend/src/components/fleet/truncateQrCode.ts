const QR_CODE_LABEL_MAX_LENGTH = 20
const ELLIPSIS = "..."

export function truncateQrCode(qrCode: string): string {
  if (qrCode.length <= QR_CODE_LABEL_MAX_LENGTH) {
    return qrCode
  }
  return qrCode.slice(0, QR_CODE_LABEL_MAX_LENGTH - ELLIPSIS.length) + ELLIPSIS
}
