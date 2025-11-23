import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Day } from '../day/day';
import { DayConfig } from '../models/day-config';

@Component({
  selector: 'app-days',
  imports: [CommonModule, Day],
  templateUrl: './days.html',
  styleUrl: './days.css',
})
export class Days {
  @Input() days: DayConfig[] = [];
}
