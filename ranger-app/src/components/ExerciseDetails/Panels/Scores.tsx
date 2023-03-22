import type React from 'react';
import PageHolder from 'src/components/PageHolder';
import {useTranslation} from 'react-i18next';
import type {Deployment} from 'src/models/deployment';
import ScoreTags from 'src/components/Deployment/ScoreTags/ScoreTags';
import {H4} from '@blueprintjs/core';
import {useNavigate} from 'react-router-dom';
import {sortByProperty} from 'sort-by-property';

const ScoresPanel = ({deployments}:
{deployments: Deployment[] | undefined;
}) => {
  const {t} = useTranslation();
  const navigate = useNavigate();

  const handleClick = (deploymentId: string) => {
    navigate(`deployments/${deploymentId}`);
  };

  if (deployments) {
    deployments = deployments.slice().sort(sortByProperty('updatedAt', 'desc'));

    return (
      <PageHolder>
        <div className='flex flex-col'>
          <table className='
              bp4-html-table
              bp4-html-table-striped
              bp4-interactive'
          >
            <tbody>
              {deployments.map(deployment => (
                <tr
                  key={deployment.id}
                  onClick={() => {
                    handleClick(deployment.id);
                  }}
                >
                  <td className='flex flex-row justify-between'>
                    <H4 className='mb-0'>{deployment.name}</H4>
                    <ScoreTags
                      exerciseId={deployment.exerciseId}
                      deploymentId={deployment.id}/>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </PageHolder>
    );
  }

  return (
    <div className='
      flex justify-center align-center m-2 mt-10 mb-auto text-gray-400'
    >
      {t('exercises.noDeployments')}
    </div>
  );
};

export default ScoresPanel;
