import { Component, signal } from '@angular/core';

import init, { greet } from 'rust_wasm';
import { Information } from './information/information';
import { Days } from './days/days';

@Component({
  selector: 'app-root',
  imports: [Information, Days],
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
