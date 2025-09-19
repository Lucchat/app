import { Injectable } from '@angular/core';
import { core } from '@tauri-apps/api';
import { LoginResponse, LoginPayload, RegisterPayload } from '../models/auth.models';

@Injectable({ providedIn: 'root' })
export class AuthService {
  async login(payload: LoginPayload): Promise<LoginResponse> {
    return core.invoke<LoginResponse>('login', { payload });
  }

  async register(payload: RegisterPayload): Promise<LoginResponse> {
    return core.invoke<LoginResponse>('register', { payload });
  }
}
