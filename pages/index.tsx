import Head from 'next/head'
import { Group, Stack, Text } from '@mantine/core'
import { DataTable } from 'mantine-datatable'
import styles from '@/styles/Home.module.css'

export default function Home() {
  let records = [
    { id: 1, name: 'Pfeffer and Sons', city: 'Tyler', state: 'MA',  streetAddress: '91877 Greenholt Cove', missionStatement: 'strategize intuitive infrastructures' },
  ];

  return (
    <>
      <Head>
        <title>Showoff!</title>
        <meta name="description" content="A lsof frontend" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={styles.main}>
        <DataTable
          className={styles.table}
          withBorder
          borderRadius="sm"
          withColumnBorders
          striped
          highlightOnHover
          records={records}
          columns={[{ accessor: 'id' }, { accessor: 'name' }, { accessor: 'city' }, { accessor: 'state' }]}
          rowExpansion={{
            content: ({ record }) => (
              <Stack p="xs" spacing={6}>
                <Group spacing={6}>
                  <Text>Postal address:</Text>
                  <Text>
                    {record.streetAddress}, {record.city}, {record.state}
                  </Text>
                </Group>
                <Group spacing={6}>
                  <Text>Mission statement:</Text>
                  <Text italic>{record.missionStatement}</Text>
                </Group>
              </Stack>
            ),
          }}
        />
      </main>
    </>
  );
}