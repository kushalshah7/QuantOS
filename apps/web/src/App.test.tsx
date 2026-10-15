import { renderToStaticMarkup } from 'react-dom/server'
import { expect, test } from 'vitest'
import App from './App'

test('the placeholder clearly discloses its stage and has no market controls', () => {
  const html = renderToStaticMarkup(<App />)

  expect(html).toContain('<h1>QuantOS</h1>')
  expect(html).toContain('Phase 0 foundation')
  expect(html).toContain('Market functionality is unavailable')
  expect(html).toContain('aria-labelledby="stage-heading"')
  expect(html).not.toMatch(/<(input|button|canvas|iframe)\b/)
})
