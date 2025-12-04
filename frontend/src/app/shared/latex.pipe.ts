import { Pipe, PipeTransform, SecurityContext } from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import katex from 'katex';

@Pipe({
  name: 'latex',
  standalone: true,
})
export class LatexPipe implements PipeTransform {
  constructor(private sanitizer: DomSanitizer) {}

  transform(value: string | null | undefined): SafeHtml {
    if (!value) {
      return '';
    }
    // Sanitize input
    const sanitizedInput = this.sanitizer.sanitize(SecurityContext.HTML, value) ?? '';

    // Replace $...$ math segments with KaTeX-rendered HTML
    const inlineMath = /\$(.+?)\$/g;
    const blockMath = /\$\$(.+?)\$\$/gs;

    let result = value;

    result = result.replace(blockMath, (org, expr: string) => {
      try {
        return katex.renderToString(expr, {
          throwOnError: true,
          displayMode: true,
        });
      } catch (err) {
        console.error('KaTeX rendering error:', err);
        return '[LATEX ERROR]';
      }
    });

    result = result.replace(inlineMath, (org, expr: string) => {
      try {
        return katex.renderToString(expr, {
          throwOnError: true,
          displayMode: false,
        });
      } catch (err) {
        console.error('KaTeX rendering error:', err);
        return '[LATEX ERROR]';
      }
    });

    // Assume that Katex is safe
    return this.sanitizer.bypassSecurityTrustHtml(result);
  }
}
