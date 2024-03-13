import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api'
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  title = 'ng-love';
}
