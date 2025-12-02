import { Component, Input, OnInit, signal, WritableSignal, effect, untracked } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DayConfig } from '../models/day-config';

interface PartState {
  running: boolean;
}

@Component({
  selector: 'app-day',
  imports: [CommonModule, FormsModule],
  templateUrl: './day.html',
  styleUrl: './day.css',
  host: {
    '[class.expanded]': 'expanded()',
  },
})
export class Day implements OnInit {
  @Input({ required: true }) config!: DayConfig;

  protected expanded = signal(false);
  protected inputData = signal('');
  protected outputPart1 = signal('');
  protected outputPart2 = signal('');
  protected durationPart1 = signal<string | null>(null);
  protected durationPart2 = signal<string | null>(null);
  private part1State: PartState = { running: false };
  private part2State: PartState = { running: false };

  constructor() {
    effect(() => {
      this.inputData();
      untracked(() => {
        void this.runPart1();
        void this.runPart2();
      });
    });
  }

  ngOnInit(): void {
    this.inputData.set(this.config.example || '');
  }

  protected dayNumber() {
    return this.config?.dayNumber ?? 0;
  }

  protected description() {
    return (
      this.config?.description ||
      'This is a placeholder for the problem description and solution notes. You can describe the algorithm used, complexity, or any interesting tricks.'
    );
  }

  protected title() {
    return `Day ${this.dayNumber()}: ${this.config.title}`;
  }

  toggle() {
    this.expanded.update((v) => !v);
    if (this.expanded() && !this.inputData()) {
      this.inputData.set(this.config.example || '');
    }
  }

  onInputChange(value: string) {
    this.inputData.set(value);
  }

  async runPart1(): Promise<void> {
    if (this.config.part1) {
      await this.executePart(
        this.config.part1,
        this.outputPart1,
        this.durationPart1,
        this.part1State
      );
    } else {
      this.outputPart1.set('Part 1 not implemented yet.');
      this.durationPart1.set(null);
    }
  }

  async runPart2(): Promise<void> {
    if (this.config.part2) {
      await this.executePart(
        this.config.part2,
        this.outputPart2,
        this.durationPart2,
        this.part2State
      );
    } else {
      this.outputPart2.set('Part 2 not implemented yet.');
      this.durationPart2.set(null);
    }
  }

  private async executePart(
    part: (input: string) => any,
    output: WritableSignal<string>,
    duration: WritableSignal<string | null>,
    state: PartState
  ): Promise<void> {
    if (state.running) {
      return;
    }

    state.running = true;
    try {
      await this.run(part, output, duration);
    } finally {
      state.running = false;
    }
  }

  private async run(
    part: (input: string) => any,
    output: WritableSignal<string>,
    duration?: WritableSignal<string | null>
  ): Promise<void> {
    const start = this.timestamp();
    try {
      const input = this.inputData().trim();
      if (!input) {
        output.set('');
        duration?.set(null);
        return;
      }
      const result = await Promise.resolve(part(input));
      output.set(String(result));
      const end = this.timestamp();
      duration?.set(this.formatDuration(end - start));
    } catch (e: any) {
      duration?.set(null);
      if (e instanceof Error) {
        if (e.name == 'RuntimeError') {
          output.set(`An error occurred during execution, please check your input data.`);
          return;
        }
        output.set(e.toString());
        return;
      }
      output.set('An unknown error occurred.');
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
