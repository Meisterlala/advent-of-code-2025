import { Component, OnInit, signal } from '@angular/core';

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

  private worker: Worker | null = null;

  async ngOnInit(): Promise<void> {
    this.worker = new Worker(new URL('./workers/solution.worker', import.meta.url));
    await this.loadDays();
  }

  private async loadDays() {
    this.loading.set(true);
    try {
      const days = await new Promise<DayConfig[]>((resolve, reject) => {
        this.worker!.onmessage = ({ data }) => {
          if (data.error) reject(new Error(data.error));
          else resolve(data.result);
        };
        this.worker!.onerror = (err) => reject(err);
        this.worker!.postMessage({ action: 'getDays' });
      });

      console.log(`Loaded from wasm worker:`, days);
      this.days.set(days);
    } catch (error) {
      console.error('Failed to load days from worker.', error);
      this.error.set(error instanceof Error ? error.message : String(error));
    } finally {
      this.loading.set(false);
    }
  }
}
