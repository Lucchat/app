import { User } from "./user.models";

export interface LoginPayload {
    username: string;
    password: string;
}

export interface RegisterPayload {
    username: string;
    password: string;
}

export interface LoginResponse {
  user: User;
  token: Tokens;
}

export interface Tokens {
  access: string;
  refresh: string;
}