const {
  buildConfig,
  buildPlayer,
  buildRunner,
} = require('./init')

const runner = buildRunner()

const config = buildConfig()

runner.registerScenario('Agent registration API', async (s, t) => {
  // load Alice and connect
  const alice = await buildPlayer(s, config, ['agent-registration-open'])
  const [alice_cell] = alice.cells
  const aliceAddr = alice.agent
  await s.consistency()

  let resp = await alice_cell.call('agent_registration', 'get_registered', null)
  t.equal(resp[0], aliceAddr, 'querying agent is included in registered agent list as they themselves are accessing')
  t.equal(resp.length, 1, 'only single agent is returned')

  resp = await alice_cell.call('agent_registration', 'get_registered', null)
  t.equal(resp.length, 1, 'agent is only recorded once')

  resp = await alice_cell.call('agent_registration', 'is_registered', { address: aliceAddr })
  t.equal(resp, true, 'can check own registration status')

  // Load Bob, but don't hit the network yet
  const bob = await buildPlayer(s, config, ['agent-registration-open'], false)
  const [bob_cell] = bob.cells
  const bobAddr = bob.agent

  resp = await alice_cell.call('agent_registration', 'is_registered', { address: bobAddr })
  t.equal(resp, false, 'can check other registration statuses')

  // Bob hits the DNA for the first time
  bob.startup()
  await s.consistency()
  resp = await bob_cell.call('agent_registration', 'get_registered', null)

  await s.consistency()

  resp = await alice_cell.call('agent_registration', 'is_registered', { address: bobAddr })
  t.equal(resp, true, 'other agents detected after they have accessed')

  resp = await alice_cell.call('agent_registration', 'get_registered', null)
  t.equal(resp.length, 2, 'new agents are recorded')
})

runner.run()
