import { Component, OnInit, signal } from '@angular/core';

import init, * as wasmExports from 'advent-of-code-2025';
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
      await init('advent_of_code_2025_bg.wasm');
      await this.loadDays();
    } catch (error) {
      console.error('Failed to initialize WebAssembly module.', error);
      this.error.set(error instanceof Error ? error.message : String(error));
    } finally {
      this.loading.set(false);
    }
  }

  private async loadDays() {
    const loadedDays: DayConfig[] = [];
    const wasm = wasmExports as any;

    // sleep to simulate delay
    // await new Promise(resolve => setTimeout(resolve, 4000));

    // Check for required exports
    const requiredExports = ['get_days'];
    for (const exp of requiredExports) {
      if (typeof wasm[exp] !== 'function') {
        console.error(`Missing required export: ${exp}`);
        throw new Error(`WASM module is missing required export: ${exp}`);
      }
    }

    // Load days
    const days: wasmExports.Day[] = wasmExports.get_days();
    for (const day of days) {
      const part1 = typeof day.part1 === 'function' ? day.part1.bind(day) : undefined;
      const part2 = typeof day.part2 === 'function' ? day.part2.bind(day) : undefined;
      loadedDays.push({
        dayNumber: day.number,
        description: day.desc,
        title: day.title,
        part1,
        part2,
      });
    }

    // Sanity check
    if (loadedDays.length === 0) {
      console.log('WASM exports:', Object.keys(wasm));
      throw new Error('No days were loaded from WASM exports.');
    }

    this.days.set(loadedDays);
  }
}
