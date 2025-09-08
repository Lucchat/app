import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { core } from '@tauri-apps/api';
import { getCurrentWindow } from '@tauri-apps/api/window';
const appWindow = getCurrentWindow();

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent {
  username: string = '';
  password: string = '';
  errorMessage: string = '';

  async onSubmit() {
    try {
      const response = await core.invoke<LoginResponse>('login', {
        payload: {
          username: this.username,
          password: this.password
        }
      });

      console.log('Réponse JWT :', response);
      localStorage.setItem('jwt', (response as any).access_token);
    } catch (err: any) {
      this.errorMessage = err.toString();
    }
  }

  minimize() {
    appWindow.minimize();
  }

  close() {
    appWindow.close();
  }
}

interface LoginResponse {
  access_token: string;
}

