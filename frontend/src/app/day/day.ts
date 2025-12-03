import { Component, Input, OnInit, signal, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DomSanitizer, SafeResourceUrl } from '@angular/platform-browser';
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
  private sanitizer = inject(DomSanitizer);

  protected expanded = signal(false);
  protected inputData = signal('');
  protected aocUrl!: SafeResourceUrl;

  ngOnInit(): void {
    this.aocUrl = this.sanitizer.bypassSecurityTrustResourceUrl(
      `https://adventofcode.com/2025/day/${this.dayNumber()}`,
    );
  }

  protected dayNumber() {
    return this.config?.dayNumber ?? 0;
  }

  protected get sourceUrl() {
    const day = this.dayNumber().toString().padStart(2, '0');
    return `https://github.com/meisterlala/advent-of-code-2025/blob/master/rust-wasm/src/day_${day}.rs`;
  }

  protected description() {
    return this.config?.description || '';
  }

  protected get isComplete() {
    return this.config?.part1 && this.config?.part2;
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
