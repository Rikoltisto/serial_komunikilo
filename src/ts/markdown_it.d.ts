declare module 'markdown-it' {
  class MarkdownIt {
    constructor(options?: any)
    render(md: string): string
  }
  export default MarkdownIt
}