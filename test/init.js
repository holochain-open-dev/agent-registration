/**
 * Test environment bootstrap
 *
 * @package Agent registration hApp
 * @author  pospi <pospi@spadgos.com>
 * @since   2020-05-05
 */

require('source-map-support').install()

const path = require('path')
const tape = require('tape')

const { Orchestrator, Config, combine, tapeExecutor, localOnly } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  console.error('unhandled rejection:', error)
  // delay exit so that debug logs have time to pipe through
  setTimeout(() => {
    process.exit(1)
  }, 500)
})

// DNA loader, to be used with `buildTestScenario` when constructing DNAs for testing
const getDNA = ((dnas) => (name) => (dnas[name]))({
  'agent-registration-open': path.resolve(__dirname, '../dnas/agent-registration-open/agent_registration_open_public_example.dna'),
  'agent-registration-invite-only': path.resolve(__dirname, '../dnas/agent-registration-invite-only/agent_registration_invite_only_private_example.dna'),
})

/**
 * Construct a test scenario out of the set of input instances & bridge configurations
 *
 * @param  {object} instances mapping of instance IDs to DNAs (@see getDNA)
 * @param  {object} bridges   (optional) mapping of bridge IDs to DNA instance ID pairs
 * @return Try-o-rama config instance for creating 'players'
 */
const buildConfig = (instances, bridges) => {
  return Config.gen(instances, {
    bridges: Object.keys(bridges || {}).reduce((b, bridgeId) => {
      b.push(Config.bridge(bridgeId, ...bridges[bridgeId]))
      return b
    }, []),
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:9000',
    },
    // logger: Config.logger(!!process.env.VERBOSE_DNA_DEBUG),
  })
}

/**
 * Create a test scenario orchestrator instance
 */

const buildRunner = () => new Orchestrator({
  middleware: combine(
    tapeExecutor(tape),
    localOnly,
  ),
})

// temporary method for RSM until conductor can interpret consistency
function shimConsistency(s) {
  s.consistency = () => new Promise((resolve, reject) => {
    setTimeout(resolve, 100)
  })
}

async function waitForPlayers(player, /*cell.*/cellId, nPlayers)
{
  while (true) {
    const stateDumpRes = await player.adminWs().dumpState({ cell_id: cellId })
    console.debug('state dump:', stateDumpRes)
    const stateDump = parseStateDump(stateDumpRes)
    console.info(`waiting for ${nPlayers} peers via player '${player.name}' in cell`, cellId)
    if (stateDump.numPeers === nPlayers - 1) {
      break
    }
    await delay(5000)
  }
}

const delay = ms => new Promise(r => setTimeout(r, ms))

// @see https://github.com/holochain/elemental-chat/blob/197bcbb860a2db52507facf3944bf35f71a0fdca/tests/src/behaviors/tx-per-second.ts#L96
const parseStateDump = ([unused, stateDumpRelevant]) => {
  const regex = /^--- Cell State Dump Summary ---\nNumber of other peers in p2p store: (\d+),\n(Ops: Limbo \(validation: (\d+) integration: (\d+)\) Integrated: (\d+)\n)?Elements authored: (\d+), Ops published: (\d+)/

  const groups = regex.exec(stateDumpRelevant)

  if (groups === null) {
    throw new Error("failed to parse state dump")
  }

  return {
    numPeers: Number.parseInt(groups[1], 10),
    opsValidationLimbo: Number.parseInt(groups[3], 10),
    opsIntegrationLimbo: Number.parseInt(groups[4], 10),
    opsIntegrated: Number.parseInt(groups[5], 10),
    elementsAuthored: Number.parseInt(groups[6], 10),
    opsPublished: Number.parseInt(groups[7], 10),
  }
}

module.exports = {
  getDNA,
  buildConfig,
  buildRunner,
  waitForPlayers,
  shimConsistency,
}
