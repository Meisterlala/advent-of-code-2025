import { DayConfig } from '../models/day-config';

export type SolveRequest = {
  action: 'solve';
  dayNumber: number;
  part: 'part1' | 'part2';
  input: string;
};

export type GetDaysRequest = {
  action: 'getDays';
};

export type WorkerRequest = SolveRequest | GetDaysRequest;

export type SolveResponse = {
  result: string;
  duration: number;
};

export type GetDaysResponse = {
  result: DayConfig[];
};

export type ErrorResponse = {
  error: string;
};

export type WorkerResponse = SolveResponse | GetDaysResponse | ErrorResponse;
