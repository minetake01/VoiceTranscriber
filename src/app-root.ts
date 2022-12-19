import { LitElement, css, html } from 'lit'
import { customElement } from 'lit/decorators.js'

@customElement('app-root')
export class MyElement extends LitElement {

  render() {
    return html`
      
    `
  }

  static styles = css`
  
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    'app-root': MyElement
  }
}
