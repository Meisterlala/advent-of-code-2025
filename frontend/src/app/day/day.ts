import { Component, Input, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DayConfig } from '../models/day-config';

@Component({
  selector: 'app-day',
  imports: [CommonModule, FormsModule],
  templateUrl: './day.html',
  styleUrl: './day.css',
  host: {
    '[class.expanded]': 'expanded()',
  },
})
export class Day {
  @Input({ required: true }) config!: DayConfig;

  protected expanded = signal(false);
  protected inputData = signal('');
  protected outputPart1 = signal('');
  protected outputPart2 = signal('');

  protected dayNumber = computed(() => this.config.dayNumber);
  protected description = computed(
    () =>
      this.config.description ||
      'This is a placeholder for the problem description and solution notes. You can describe the algorithm used, complexity, or any interesting tricks.'
  );

  toggle() {
    this.expanded.update((v) => !v);
  }

  runPart1() {
    if (this.config.part1) {
      try {
        const result = this.config.part1(this.inputData());
        this.outputPart1.set(String(result));
      } catch (e: any) {
        this.outputPart1.set('Error: ' + e.toString());
      }
    } else {
      this.outputPart1.set('Part 1 not implemented yet.');
    }
  }

  runPart2() {
    if (this.config.part2) {
      try {
        const result = this.config.part2(this.inputData());
        this.outputPart2.set(String(result));
      } catch (e: any) {
        this.outputPart2.set('Error: ' + e.toString());
      }
    } else {
      this.outputPart2.set('Part 2 not implemented yet.');
    }
  }
}
