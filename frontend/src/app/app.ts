import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';

import init, { greet } from 'rust_wasm';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.html',
  styleUrl: './app.css',
})
export class App {
  protected readonly title = signal('frontend');

  async ngOnInit(): Promise<void> {
    try {
      await init('/rust_wasm_bg.wasm');
    } catch (error) {
      console.error('Failed to initialize WebAssembly module.', error);
    }

    const message = greet('Adventurer');
    console.log(message);
  }
}
