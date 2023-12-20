import React, {useState, useEffect} from 'react';
import {DateTime} from 'luxon';
import {type DeploymentEvent} from 'src/models/exercise';
import {useTranslation} from 'react-i18next';

const ProgressBarWithTimer = ({event}: {event: DeploymentEvent}) => {
  const {t} = useTranslation();
  const [timeLeft, setTimeLeft] = useState('');

  useEffect(() => {
    const timer = setInterval(() => {
      const now = DateTime.utc();
      const end = DateTime.fromISO(event.end, {zone: 'UTC'});
      const duration = end.diff(now);

      setTimeLeft(duration.toFormat('hh:mm:ss'));
    }, 1000);

    return () => {
      clearInterval(timer);
    };
  }, [event]);

  const now = DateTime.utc();
  const end = DateTime.fromISO(event.end, {zone: 'UTC'});
  const start = DateTime.fromISO(event.start, {zone: 'UTC'});
  const totalDuration = end.diff(start, 'milliseconds').milliseconds;
  const elapsed = now.diff(start, 'milliseconds').milliseconds;
  const progress = Math.min(100, (elapsed / totalDuration) * 100);
  const futureStart = start.diff(now);

  return (
    <div className='w-full mb-2 h-4 bg-gray-200 rounded-full relative'>
      {now < start ? (
        <div
          className='flex absolute top-0 left-0 w-full h-full items-center justify-center
          text-sm'
        >
          {t('deployments.events.eventWillOpenIn')} {futureStart.toFormat('hh:mm:ss')}
        </div>
      ) : (now < end ? (
        <>
          <div style={{width: `${progress}%`}} className='h-4 bg-blue-500 rounded-full'/>
          <div
            className={`flex absolute top-0 left-0 w-full h-full items-center justify-center 
            text-sm font-bold ${progress > 50 ? 'text-white' : 'text-black'}`}
          >
            {timeLeft}
          </div>
        </>
      ) : (
        <div
          className='flex absolute top-0 left-0 w-full h-full items-center justify-center
          text-sm'
        >
          {t('deployments.events.eventWindowClosed')}
        </div>
      ))}
    </div>
  );
};

export default ProgressBarWithTimer;
