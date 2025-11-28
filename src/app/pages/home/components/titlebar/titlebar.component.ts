import { Component, OnInit, OnDestroy } from '@angular/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

@Component({
  selector: 'app-titlebar',
  standalone: true,
  templateUrl: './titlebar.component.html',
  styleUrl: './titlebar.component.css'
})
export class TitlebarComponent implements OnInit, OnDestroy {
  private unlisten: (() => void) | null = null;

  async ngOnInit() {
    // Quand la fenêtre est redimensionnée
    this.unlisten = await appWindow.onResized(async () => {
      const maximized = await appWindow.isMaximized();
      const winEl = document.querySelector('.window');
      if (winEl) {
        if (maximized) {
          winEl.classList.add('maximized');
        } else {
          winEl.classList.remove('maximized');
        }
      }
    });
  }

  ngOnDestroy() {
    if (this.unlisten) this.unlisten();
  }

  minimize() { appWindow.minimize(); }
  close() { appWindow.close(); }
  maximize() {
    appWindow.isMaximized().then(maximized => {
      if (maximized) {
        appWindow.unmaximize();
      } else {
        appWindow.maximize();
      }
    });
  }
}
