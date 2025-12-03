import { Component, OnInit, signal } from '@angular/core';

import { Information } from './information/information';
import { Days } from './days/days';
import { DayConfig } from './models/day-config';
import { GetDaysRequest, WorkerResponse } from './workers/worker.types';

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
        if (!this.worker) return reject(new Error('Worker not initialized'));

        this.worker.onmessage = ({ data }: { data: WorkerResponse }) => {
          if ('error' in data) {
            reject(new Error(data.error));
          } else if ('result' in data && Array.isArray(data.result)) {
            resolve(data.result);
          } else {
            reject(new Error('Unexpected response from worker'));
          }
        };
        this.worker.onerror = (err) => reject(err);

        const request: GetDaysRequest = { action: 'getDays' };
        this.worker.postMessage(request);
      });

      console.log(`Days loaded from wasm worker:`, days);
      this.days.set(days);
    } catch (error) {
      console.error('Failed to load days from worker.', error);
      this.error.set(error instanceof Error ? error.message : String(error));
    } finally {
      this.loading.set(false);
      if (this.worker) {
        this.worker.terminate();
        this.worker = null;
      }
    }
  }
}
