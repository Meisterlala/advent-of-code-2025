export interface DayConfig {
  dayNumber: number;
  title: string;
  description: string;
  example: string;
  part1?: (input: string) => any;
  part2?: (input: string) => any;
}
