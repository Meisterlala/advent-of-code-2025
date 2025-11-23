import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';

import init, * as wasmExports from 'rust_wasm';
import { Information } from './information/information';
import { Days } from './days/days';
import { DayConfig } from './models/day-config';

@Component({
  selector: 'app-root',
  imports: [Information, Days],
  templateUrl: './app.html',
  styleUrl: './app.css',
})
export class App {
  protected days = signal<DayConfig[]>([]);

  async ngOnInit(): Promise<void> {
    try {
      await init('/rust_wasm_bg.wasm');
      this.loadDays();
    } catch (error) {
      console.error('Failed to initialize WebAssembly module.', error);
    }
  }

  private loadDays() {
    const loadedDays: DayConfig[] = [];
    const wasm = wasmExports as any;

    console.log('WASM exports:', Object.keys(wasm));

    for (let i = 1; i <= 25; i++) {
      // Check for day{i}_part1, day{i}_part2, day{i}_desc
      // Adjust naming convention as needed based on Rust exports
      const part1Func = wasm[`day${i}_part1`];
      const part2Func = wasm[`day${i}_part2`];
      const descFunc = wasm[`day${i}_desc`];

      // If at least one part exists, we consider the day "available"
      if (part1Func || part2Func) {
        loadedDays.push({
          dayNumber: i,
          description: descFunc ? descFunc() : undefined, // Let Day component handle default
          part1: part1Func,
          part2: part2Func,
        });
      }
    }
    
    // Debug
    for (let i =0; i <10; i++) {
      loadedDays.push({
        dayNumber: i + 1,
        description: `Description for Day ${i + 1}`,
        part1: (input: string) => `Result of Part 1 for Day ${i + 1} with input length ${input.length}`,
        part2: (input: string) => `Result of Part 2 for Day ${i + 1} with input length ${input.length}`,
      });
    }

    this.days.set(loadedDays);
  }
}
