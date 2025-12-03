import { Component, input, OnDestroy, OnInit, signal, effect, untracked } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { SolveRequest, WorkerResponse } from '../workers/worker.types';

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
  private timeoutId: ReturnType<typeof setTimeout> | null = null;

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

  private clearTimeoutIfAny() {
    if (this.timeoutId !== null) {
      clearTimeout(this.timeoutId);
      this.timeoutId = null;
    }
  }

  // TODO: Refactor this whole thing, there should be a cleaner way to do this.
  private async run(input: string): Promise<void> {
    const trimmedInput = input.trim();

    // If input is empty, clear output and return
    if (!trimmedInput) {
      this.clearTimeoutIfAny();
      this.output.set('');
      this.duration.set(null);
      return;
    }

    this.clearTimeoutIfAny();
    // If the worker is currently running a calculation, we must terminate it
    // to cancel the previous operation, as WASM is blocking.
    this.terminateWorker();

    // If worker doesn't exist (was terminated or not created), create it
    if (!this.worker) {
      this.createWorker();
    }

    this.running.set(true);

    try {
      const { result, duration } = await this.solveInWorker(trimmedInput);
      this.output.set(result);
      this.duration.set(this.formatDuration(duration));
    } catch (e: any) {
      this.handleError(e);
    } finally {
      this.running.set(false);
      this.clearTimeoutIfAny();
    }
  }

  private solveInWorker(input: string): Promise<{ result: string; duration: number }> {
    return new Promise((resolve, reject) => {
      if (!this.worker) return reject(new Error('Worker not initialized'));

      this.timeoutId = setTimeout(() => {
        this.terminateWorker();
        this.timeoutId = null;
        reject(new Error(`Calculation timed out after ${this.TIMEOUT_MS / 1000} seconds`));
      }, this.TIMEOUT_MS);

      this.worker.onmessage = ({ data }: { data: WorkerResponse }) => {
        this.clearTimeoutIfAny();
        if ('error' in data) {
          reject(new Error(data.error));
        } else if ('result' in data && 'duration' in data) {
          resolve(data as { result: string; duration: number });
        } else {
          reject(new Error('Unexpected response from worker'));
        }
      };

      this.worker.onerror = (err) => {
        this.clearTimeoutIfAny();
        reject(err);
      };

      const request: SolveRequest = {
        action: 'solve',
        dayNumber: this.dayNumber(),
        part: this.part(),
        input,
      };
      this.worker.postMessage(request);
    });
  }

  private handleError(e: any) {
    this.duration.set(null);

    if (e.message && e.message.includes('Worker')) {
      this.output.set('An error occurred in the worker thread.');
      return;
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

  protected recalculate() {
    if (!this.running() && this.duration()) {
      void this.run(this.puzzleInput());
    }
  }
}
