import {Button, ButtonGroup} from '@blueprintjs/core';
import React, {useState} from 'react';
import Accordion from 'src/components/Accordion';
import AccordionGroup from 'src/components/AccordionGroup';
import PageHolder from 'src/components/PageHolder';
import {type DummyManualMetric} from 'src/models/manualMetric';
import {ExerciseRole, ExerciseRoleOrder} from 'src/models/scenario';
import {getRoleColor} from 'src/utils';
import {t} from 'i18next';
import MetricScoringForm from './MetricScoringForm';

const MetricScorer = ({exerciseId, deploymentId}:
{
  exerciseId: string;
  deploymentId: string;
}) => {
  const [selectedRole, setSelectedRole] = useState<ExerciseRole | undefined>(undefined);

  const handleRoleButtonClick = (role: ExerciseRole) => {
    setSelectedRole(role);
  };

  // Get the manualMetrics from the current deployment
  let dummyMetricObjects: DummyManualMetric[] | undefined;
  // eslint-disable-next-line prefer-const
  dummyMetricObjects = [
    {
      id: '0',
      name: 'Some Metric Name',
      maxScore: 100,
      role: ExerciseRole.Blue,
      textSubmissionValue: 'This is a very long text submission voluptatem, quia voluptas sit, aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos, qui ratione voluptatem sequi nesciunt, neque porro quisquam est, qui dolorem ipsum, quia dolor sit amet consectetur adipisci[ng] velit, sed quia non numquam [do] eius modi tempora inci[di]dunt, ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum[d] exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? [D]Quis autem vel eum i[r]ure reprehenderit, qui in ea voluptate velit esse, quam nihil molestiae consequatur, vel illum, qui dolorem eum fugiat, quo voluptas nulla pariatur? [33] At vero eos et accusamus et iusto odio dignissimos ducimus, qui blanditiis praesentium voluptatum deleniti atque corrupti, quos dolores et quas molestias excepturi sint, obcaecati cupiditate non provi', // eslint-disable-line max-len
    },
    {
      id: '1',
      name: 'My Cool Metric',
      maxScore: 50,
      role: ExerciseRole.Blue,
    },
    {
      id: '2',
      name: 'My Other Cool Metric',
      maxScore: 25,
      role: ExerciseRole.Red,
      textSubmissionValue: 'This is a participant text submission',
    },
  ];

  if (dummyMetricObjects) {
    const defaultRole = dummyMetricObjects[0].role;
    if (!selectedRole) {
      setSelectedRole(defaultRole);
    }

    const filteredArtifacts = selectedRole
      ? dummyMetricObjects.filter(artifact => artifact.role === selectedRole)
      : dummyMetricObjects.filter(artifact => artifact.role === defaultRole);

    const availableRoles
    = Array.from(new Set(dummyMetricObjects.map(artifact => artifact.role)))
      .sort((roleA, roleB) => ExerciseRoleOrder[roleA] - ExerciseRoleOrder[roleB]);

    return (
      <PageHolder>
        <div className='flex justify-center space-x-4'>
          <ButtonGroup fill>
            {availableRoles.map(role => (
              <Button
                key={role}
                style={{backgroundColor: selectedRole === role ? getRoleColor(role) : 'gainsboro'}}
                active={role === selectedRole}
                className='rounded-full mb-4'
                onClick={() => {
                  handleRoleButtonClick(role);
                }}
              >
                <span className='font-bold text-white text-'> {role} {t('common.team')} </span>
              </Button>
            ))}
          </ButtonGroup>
        </div>
        <AccordionGroup>
          {filteredArtifacts.map(artifact => (
            <Accordion
              key={artifact.id}
              className='mb-4 p-2 border-2 border-slate-300 shadow-md '
              title={artifact.name}
            >
              <MetricScoringForm
                metric={artifact}
                onSubmit={artifactScorerForm => {
                  // PUT mutation for artifact scoring here
                  // await the update and create a toast
                  console.log(artifactScorerForm);
                }}/>
            </Accordion>
          ))}
        </AccordionGroup>
      </PageHolder>
    );
  }

  return (
    <PageHolder>
      <div className='
    flex justify-center align-center m-2 mt-auto mb-4 text-gray-400'
      >
        {t('metricScoring.noManualMetrics')}
      </div>

    </PageHolder>
  );
};

export default MetricScorer;
