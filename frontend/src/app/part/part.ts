import { Component, input, OnDestroy, OnInit, signal, effect, untracked } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-part',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './part.html',
  styleUrl: './part.css',
})
export class Part implements OnDestroy, OnInit {
  dayNumber = input.required<number>();
  part = input.required<'part1' | 'part2'>();
  puzzleInput = input.required<string>();

  protected output = signal('');
  protected duration = signal<string | null>(null);
  protected running = signal(false);

  private worker: Worker | null = null;
  private readonly TIMEOUT_MS = 10000;

  constructor() {
    effect(() => {
      // Track input changes
      const currentInput = this.puzzleInput();
      untracked(() => {
        void this.run(currentInput);
      });
    });
  }

  ngOnInit(): void {
    this.createWorker();
  }

  ngOnDestroy(): void {
    this.terminateWorker();
  }

  private createWorker() {
    this.terminateWorker();
    this.worker = new Worker(new URL('../workers/solution.worker', import.meta.url));
    this.running.set(false);
  }

  private terminateWorker() {
    if (this.worker) {
      this.worker.terminate();
      this.worker = null;
      this.running.set(false);
    }
  }

  private async run(input: string): Promise<void> {
    const trimmedInput = input.trim();

    if (!trimmedInput) {
      this.output.set('');
      this.duration.set(null);
      return;
    }

    // If the worker is currently running a calculation, we must terminate it
    // to cancel the previous operation, as WASM is blocking.
    if (this.running()) {
      this.terminateWorker();
    }

    // If worker doesn't exist (was terminated or not created), create it
    if (!this.worker) {
      this.createWorker();
    }

    this.running.set(true);
    const start = this.timestamp();

    try {
      const resultPromise = new Promise<{ result: string; duration: number }>((resolve, reject) => {
        if (!this.worker) return reject(new Error('Worker not initialized'));

        this.worker.onmessage = ({ data }) => {
          if (data.error) {
            reject(new Error(data.error));
          } else {
            resolve(data);
          }
        };

        this.worker.onerror = (err) => reject(err);

        this.worker.postMessage({
          action: 'solve',
          dayNumber: this.dayNumber(),
          part: this.part(),
          input: trimmedInput,
        });
      });

      const timeoutPromise = new Promise<never>((_, reject) =>
        setTimeout(() => {
          reject(new Error(`Calculation timed out after ${this.TIMEOUT_MS / 1000} seconds`));
        }, this.TIMEOUT_MS)
      );

      const { result, duration } = await Promise.race([resultPromise, timeoutPromise]);

      this.output.set(String(result));
      this.duration.set(this.formatDuration(duration));
      this.running.set(false);
    } catch (e: any) {
      this.duration.set(null);
      this.running.set(false);

      if (e.message && e.message.includes('Worker')) {
        this.output.set('An error occurred in the worker thread.');
        return;
      }

      if (e.message === `Calculation timed out after ${this.TIMEOUT_MS / 1000} seconds`) {
        this.terminateWorker();
      }

      if (e instanceof Error) {
        if (e.name === 'RuntimeError') {
          this.output.set(`An error occurred during execution, please check your input data.`);
          return;
        }
        this.output.set(e.toString());
        return;
      }
      this.output.set('An unknown error occurred.');
    }
  }

  private timestamp(): number {
    return typeof performance !== 'undefined' ? performance.now() : Date.now();
  }

  private formatDuration(durationMs: number): string {
    if (!isFinite(durationMs) || durationMs < 0) {
      return '';
    }
    if (durationMs < 1) {
      return `<1 ms`;
    }
    const rounded = Math.round(durationMs);
    return `${rounded} ms`;
  }
}
