import { Component } from '@angular/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { BackgroundComponent } from './components/background/background.component';
import { TitlebarComponent } from './components/titlebar/titlebar.component';
import { SidebarComponent } from './components/sidebar/sidebar.component';
import { ChatComponent } from './components/chat/chat.component';

const appWindow = getCurrentWindow();

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [BackgroundComponent, TitlebarComponent, SidebarComponent, ChatComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent {
  selectedContact: number | null = null;

  async ngOnInit() {
    await appWindow.maximize();
    await appWindow.setTitle("Lucchat - Home");
  }

}
