import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Day } from '../day/day';

@Component({
  selector: 'app-days',
  imports: [CommonModule, Day],
  templateUrl: './days.html',
  styleUrl: './days.css',
})
export class Days {
  days = Array.from({ length: 25 }, (_, i) => i + 1);
}
