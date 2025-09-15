import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { core } from '@tauri-apps/api';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { BackgroundComponent } from '../background/background.component';
import { LoginCompletedComponent } from '../login-completed/login-completed.component';

const appWindow = getCurrentWindow();

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [CommonModule, FormsModule, BackgroundComponent, LoginCompletedComponent],
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent {
  username = '';
  password = '';
  errorMessage = '';
  showPassword = false;
  loginSuccess = false;
  isRegisterMode = false;
  errorTimeout: any;

  async onSubmit() {
    if (this.isRegisterMode) {
      try {
        const response = await core.invoke<LoginResponse>('register', {
          payload: { username: this.username, password: this.password }
        });
        console.log('User created:', response);
        this.errorMessage = '';
        this.isRegisterMode = false;
        // utilisateur a réussi à s'enregistrer, on peut tenter de le connecter directement
        this.loginSuccess = true;
      } catch (err: any) {
        this.handleError(err, true);
      }
    } else {
      try {
        const response = await core.invoke<LoginResponse>('login', {
          payload: { username: this.username, password: this.password }
        });

        console.log(response);
        this.errorMessage = '';
        // utilisateur a réussi à se connecter
        this.loginSuccess = true;
      } catch (err: any) {
        this.handleError(err, false);
      }
    }
  }

  private handleError(err: any, isRegister: boolean) {
    const message = this.formatError(err, isRegister);
    this.errorMessage = message;

    if (this.errorTimeout) clearTimeout(this.errorTimeout);

    this.errorTimeout = setTimeout(() => {
      const el = document.querySelector('.error');
      if (el) {
        el.classList.add('fade-out');
        setTimeout(() => {
          this.errorMessage = '';
        }, 600);
      } else {
        this.errorMessage = '';
      }
    }, 6000);

    const raw = err?.status || err?.code || err?.toString();
    if ((!isRegister && raw.toString().includes("401")) || (isRegister && raw.toString().includes("409"))) {
      this.triggerShake();
    }
  }

  private triggerShake() {
    const win = document.querySelector('.window');
    if (win) {
      win.classList.add('shake');
      setTimeout(() => win.classList.remove('shake'), 500);
    }
  }

  private formatError(err: any, isRegister: boolean): string {
    const raw = err?.status || err?.code || err?.toString();
    console.log(raw)
    if (!isRegister) {
      if (raw.toString().includes("401")) {
        return "❌ Invalid username or password.";
      }
      if (raw.toString().includes("404")) {
        return "❌ Server not reachable. Please check your connection.";
      }
    } else {
      if (raw.toString().includes("409")) {
        return "❌ This username already exists. Please choose another one.";
      }
      if (raw.toString().includes("404")) {
        return "❌ Server not reachable. Please check your connection.";
      }
      if (raw.toString().includes("400")) {
        return "❌ Password too weak. It must be at least 12 characters long and include an uppercase letter, a lowercase letter, a number, and a special character.";
      }
    }

    return "❌ An unexpected error occurred. Please try again.";
  }

  toggleMode() {
    this.isRegisterMode = !this.isRegisterMode;
    this.errorMessage = '';
    this.username = '';
    this.password = '';
  }

  minimize() { appWindow.minimize(); }
  close() { appWindow.close(); }
}

interface LoginResponse {
  user: User;
  token: Tokens;
}

interface User {
  uuid: string;
  username: string;
  descriptions: string|null;
  profile_picture: string|null;
  keys: Keys;
  pending_friend_requests: string[];
  friend_requests: string[];
  friends: string[];
  token: Tokens;
}

interface Keys {
  ik_pub: Uint8Array[];
  spk_pub: Uint8Array[];
  opk_pub: Uint8Array[][];
}

interface Tokens {
  access: string;
  refresh: string;
}
