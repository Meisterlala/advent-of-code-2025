import { Component, OnInit, signal } from '@angular/core';

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
export class App implements OnInit {
  protected days = signal<DayConfig[]>([]);
  protected loading = signal(true);
  protected error = signal<string | null>(null);

  async ngOnInit(): Promise<void> {
    try {
      await init('/rust_wasm_bg.wasm');
      await this.loadDays();
      this.loading.set(false);
    } catch (error) {
      console.error('Failed to initialize WebAssembly module.', error);
      this.error.set(error instanceof Error ? error.message : String(error));
      this.loading.set(false);
    }
  }

  private async loadDays() {
    const loadedDays: DayConfig[] = [];
    const wasm = wasmExports as any;

    // sleep to simulate delay
    // await new Promise(resolve => setTimeout(resolve, 4000));

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
    // for (let i = 0; i < 10; i++) {
    //   loadedDays.push({
    //     dayNumber: i + 1,
    //     description: `Description for Day ${i + 1}`,
    //     part1: (input: string) =>
    //       `Result of Part 1 for Day ${i + 1} with input length ${input.length}`,
    //     part2: (input: string) =>
    //       `Result of Part 2 for Day ${i + 1} with input length ${input.length}`,
    //   });
    // }

    // Sanity check
    if (loadedDays.length === 0) {
      console.log('WASM exports:', Object.keys(wasm));
      throw new Error('No days were loaded from WASM exports.');
    }

    this.days.set(loadedDays);
  }
}
