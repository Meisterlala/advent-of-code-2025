export interface DayConfig {
  dayNumber: number;
  title: string;
  description: string;
  part1?: (input: string) => any;
  part2?: (input: string) => any;
}
