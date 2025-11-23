import { Component, Input, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

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
  @Input({ required: true }) dayNumber!: number;
  @Input() description =
    'This is a placeholder for the problem description and solution notes. You can describe the algorithm used, complexity, or any interesting tricks.';

  protected expanded = signal(false);
  protected inputData = signal('');
  protected outputPart1 = signal('');
  protected outputPart2 = signal('');

  toggle() {
    this.expanded.update((v) => !v);
  }
}
