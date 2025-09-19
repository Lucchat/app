import { AppError } from './types';

export function mapGenericError(err: any): AppError {
  const raw = err?.status || err?.code || err?.toString();

  if (raw.includes("Private keys not found")) {
    return { code: "NO_KEYS", message: "❌ Private keys not found. Strange situation, we need to create every keys and update database + delete every conversion and make sure his friends delete the conversation with him." };
  }

  if (raw.includes("Failed to read") || raw.includes("Failed to write") || raw.includes("Filesystem error")) {
    return { code: "FS_ERROR", message: "❌ Unable to access local files. Please check permissions." };
  }

  if (raw.includes("parse") || raw.includes("JSON")) {
    return { code: "JSON_ERROR", message: "❌ Failed to process application data. Please try again." };
  }

  if (raw.includes("ECONNREFUSED") || raw.includes("Connection refused")) {
    return { code: "CONN_REFUSED", message: "❌ Cannot reach the server. Is it running?" };
  }

  if (raw.includes("timeout")) {
    return { code: "TIMEOUT", message: "❌ The request timed out. Please try again." };
  }

  return { code: "UNKNOWN", message: "❌ An unexpected error occurred. Please try again." };
}
