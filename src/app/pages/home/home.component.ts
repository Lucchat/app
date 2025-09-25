import { Component } from '@angular/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent {

  async ngOnInit() {
    await appWindow.maximize();
    await appWindow.setTitle("Lucchat - Home");
  }

}
