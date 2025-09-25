import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { getCurrentWindow } from '@tauri-apps/api/window';

import { BackgroundComponent } from './components/background/background.component';
import { CompletedComponent } from './components/completed/completed.component';
import { AuthService } from '../../services/auth.service';
import { LoginResponse } from '../../models/auth.models';

import { mapLoginError } from '../../utils/errors/http/login-error-mapper';
import { mapRegisterError } from '../../utils/errors/http/register-error-mapper';
import { mapGenericError } from '../../utils/errors/generic-error-mapper';
import { Router } from '@angular/router';

const appWindow = getCurrentWindow();

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [CommonModule, FormsModule, BackgroundComponent, CompletedComponent],
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
  loading = false;
  loginResponse: LoginResponse | null = null;

  async ngOnInit() {
    await appWindow.setTitle("Lucchat - Login");
  }

  constructor(private auth: AuthService, private router: Router) { }

  async onSubmit() {
    if (this.loading) return;
    this.loading = true;

    try {
      if (this.isRegisterMode) {
        const response = await this.auth.register({ username: this.username, password: this.password });
        this.loginResponse = response;
        console.log('User registered:', this.loginResponse);
        this.errorMessage = '';
        this.isRegisterMode = false;
        this.loginSuccess = true;

        await new Promise(resolve => setTimeout(resolve, 1500));
        this.router.navigate(['/home']); 

      } else {
        const response = await this.auth.login({ username: this.username, password: this.password });
        this.loginResponse = response;
        console.log('User logged in:', this.loginResponse);
        this.errorMessage = '';
        this.loginSuccess = true;

        await new Promise(resolve => setTimeout(resolve, 1500));
        this.router.navigate(['/home']); 
      }
    } catch (err: any) {
      this.handleError(err, this.isRegisterMode);
    } finally {
      this.loading = false;
    }
  }

  private handleError(err: any, isRegister: boolean) {
    let appError;

    appError = isRegister ? mapRegisterError(err) : mapLoginError(err);

    if (appError.code === "UNKNOWN") {
      appError = mapGenericError(err);
    }

    this.errorMessage = appError.message;

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

    if (["401", "409"].includes(appError.code)) {
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

  toggleMode() {
    this.isRegisterMode = !this.isRegisterMode;
    this.errorMessage = '';
    this.username = '';
    this.password = '';
  }

  minimize() { appWindow.minimize(); }
  close() { appWindow.close(); }
}
