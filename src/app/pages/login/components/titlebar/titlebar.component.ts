import { Component } from '@angular/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

@Component({
  selector: 'app-titlebar',
  standalone: true,
  imports: [],
  templateUrl: './titlebar.component.html',
  styleUrl: './titlebar.component.css'
})
export class TitlebarComponent {
  minimize() { appWindow.minimize(); }
  close() { appWindow.close(); }
}
