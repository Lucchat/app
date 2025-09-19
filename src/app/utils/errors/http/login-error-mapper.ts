import { AppError } from '../types';

export function mapLoginError(err: any): AppError {
  const raw = err?.status || err?.code || err?.toString();

  if (raw.includes("401")) {
    return { code: "401", message: "❌ Invalid username or password." };
  }
  if (raw.includes("404")) {
    return { code: "404", message: "❌ Server not reachable. Please check your connection." };
  }

  return { code: "UNKNOWN", message: "❌ An unexpected error occurred during login." };
}
