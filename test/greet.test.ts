import { invoke } from '@tauri-apps/api/core'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { greet } from '../src/greet'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('greet', () => {
  beforeEach(() => {
    vi.mocked(invoke).mockReset()
  })

  it('calls the greet command with the given name', async () => {
    vi.mocked(invoke).mockResolvedValue('Hello, Tauri!')

    await expect(greet('Tauri')).resolves.toBe('Hello, Tauri!')
    expect(invoke).toHaveBeenCalledWith('greet', { name: 'Tauri' })
  })
})
