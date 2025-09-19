import { AppError } from '../types';

export function mapRegisterError(err: any): AppError {
  const raw = err?.status || err?.code || err?.toString();

  if (raw.includes("409")) {
    return { code: "409", message: "❌ This username already exists. Please choose another one." };
  }
  if (raw.includes("404")) {
    return { code: "404", message: "❌ Server not reachable. Please check your connection." };
  }
  if (raw.includes("400")) {
    return { code: "400", message: "❌ Password too weak. It must be at least 12 characters long and include an uppercase letter, a lowercase letter, a number, and a special character." };
  }

  return { code: "UNKNOWN", message: "❌ An unexpected error occurred during registration." };
}
