import { Component, Input, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DayConfig } from '../models/day-config';
import { Part } from '../part/part';

@Component({
  selector: 'app-day',
  imports: [CommonModule, FormsModule, Part],
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

  // Update input data when textarea changes
  onInputChange(value: string) {
    this.inputData.set(value);
  }
}
