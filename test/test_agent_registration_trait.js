const {
  getDNA,
  buildConfig,
  buildRunner,
  shimConsistency,
} = require('./init')

const runner = buildRunner()

const config = buildConfig()

const TEST_DNAS = ['agent-registration-open']

runner.registerScenario('Agent registration API', async (scenario, t) => {
  shimConsistency(scenario)

  const [alice, bob] = await scenario.players([config, config], false)

  // load Alice and connect
  await alice.startup()
  const [[alice_happ]] = await alice.installAgentsHapps([[TEST_DNAS.map(getDNA)]])
  const [alice_cell] = alice_happ.cells
  const aliceAddr = alice_happ.agent
  await scenario.consistency()

  let resp = await alice_cell.call('agent_registration', 'get_registered', null)
  t.deepEqual(resp[0], aliceAddr, 'querying agent is included in registered agent list as they themselves are accessing')
  t.equal(resp.length, 1, 'only single agent is returned')

  resp = await alice_cell.call('agent_registration', 'get_registered', null)
  t.equal(resp.length, 1, 'agent is only recorded once')

  resp = await alice_cell.call('agent_registration', 'is_registered', { pubKey: aliceAddr })
  t.equal(resp, true, 'can check own registration status')

  // Bob installs the hApp and hits the DNA for the first time
  await bob.startup()
  const [[bob_happ]] = await bob.installAgentsHapps([[TEST_DNAS.map(getDNA)]])
  const [bob_cell] = bob_happ.cells
  const bobAddr = bob_happ.agent
  await scenario.consistency()

  resp = await alice_cell.call('agent_registration', 'is_registered', { pubKey: bobAddr })
  t.equal(resp, true, 'other agents detected after they have accessed')

  resp = await alice_cell.call('agent_registration', 'get_registered', null)
  t.equal(resp.length, 2, 'new agents are recorded')
})

runner.run()
