/// <reference lib="webworker" />

import init, { get_days, get_day, InitOutput } from 'advent-of-code-2025';
import { DayConfig } from '../models/day-config';

let initPromise: Promise<InitOutput> | null = null;

async function ensureInit() {
  if (!initPromise) {
    initPromise = init('advent_of_code_2025_bg.wasm');
  }
  return initPromise;
}

addEventListener('message', async ({ data }) => {
  const { action, dayNumber, part, input } = data;

  try {
    await ensureInit();

    if (action === 'getDays') {
      postMessage({ result: getDays() });
      return;
    }

    if (action === 'solve') {
      const { result, duration } = solveDay(dayNumber, part, input);
      postMessage({ result, duration });
      return;
    }

    throw new Error(`Unknown action: ${action}`);
  } catch (error) {
    postMessage({ error: error instanceof Error ? error.message : String(error) });
  }
});

function solveDay(
  dayNumber: number,
  part: 'part1' | 'part2',
  input: string
): { result: string; duration: number } {
  const day = get_day(dayNumber);
  if (!day) {
    throw new Error(`Day ${dayNumber} not found`);
  }
  // Sleep for 2 seconds to simulate long computation

  const start = performance.now();
  let result: string;
  if (part === 'part1') {
    result = day.part1(input);
  } else {
    result = day.part2(input);
  }
  const end = performance.now();

  day.free();
  return { result, duration: end - start };
}

function getDays(): DayConfig[] {
  const days = get_days();
  const result: DayConfig[] = days.map((day) => ({
    dayNumber: day.number,
    title: day.title,
    description: day.desc,
    example: day.example,
    part1: true,
    part2: true,
  }));
  days.forEach((d) => d.free());
  return result;
}
