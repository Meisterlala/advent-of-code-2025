/// <reference lib="webworker" />

import init, { get_days, get_day, InitOutput } from 'advent-of-code-2025';
import { DayConfig } from '../models/day-config';
import { WorkerRequest } from './worker.types';

let initPromise: Promise<InitOutput> | null = null;

async function ensureInit() {
  if (!initPromise) {
    initPromise = init({ module_or_path: 'advent_of_code_2025_bg.wasm' });
  }
  return initPromise;
}

addEventListener('message', async ({ data }: { data: WorkerRequest }) => {
  try {
    await ensureInit();

    switch (data.action) {
      case 'getDays':
        postMessage({ result: getDays() });
        break;
      case 'solve':
        const { result, duration } = solveDay(data.dayNumber, data.part, data.input);
        postMessage({ result, duration });
        break;
      default:
        throw new Error(`Unknown action: ${(data as any).action}`);
    }
  } catch (error) {
    postMessage({
      error: error instanceof Error ? error.message : String(error),
    });
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

  const start = performance.now();
  let result: string;
  try {
    if (part === 'part1') {
      result = day.part1(input);
    } else {
      result = day.part2(input);
    }
    day.free();
  } catch (e) {
    if (e instanceof Error) {
      if (e.name === 'RuntimeError') {
        throw new Error(
          `The rust code encountered a panic. Please check your input data and try again. Details: ${e.message}`
        );
      }
      throw new Error(
        `Error solving Day ${dayNumber} ${part == 'part1' ? 'Part 1' : 'Part 2'}: ${e.message}`
      );
    }
    throw new Error(
      `Unknown error solving Day ${dayNumber} ${part == 'part1' ? 'Part 1' : 'Part 2'}`
    );
  }
  const end = performance.now();
  console.debug(
    `[Worker] Solved Day ${dayNumber} ${part == 'part1' ? 'Part 1' : 'Part 2'} in ${end - start} ms`
  );

  return { result, duration: end - start };
}

function getDays(): DayConfig[] {
  const days = get_days();
  try {
    return days.map((day) => ({
      dayNumber: day.number,
      title: day.title,
      description: day.desc,
      example: day.example,
      part1: true,
      part2: true,
    }));
  } finally {
    days.forEach((d) => d.free());
  }
}
