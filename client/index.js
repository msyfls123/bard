import wasm from './Cargo.toml';

wasm().then(({ run_app }) => {
  run_app();
});

const cos = new COS({
  getAuthorization: async (options, callback) => {
    const url = 'https://void.ebichu.cc/sts';
    let data;
    try {
      data = await fetch(url).then(res => res.json());
    } catch {}
    const { credentials, startTime, expiredTime } = data || {};
    if (!(data && credentials)) {
      return console.error('data invalid:\n' + JSON.stringify(data, null, 2))
    };
    callback({
      TmpSecretId: credentials.tmpSecretId,
      TmpSecretKey: credentials.tmpSecretKey,
      SecurityToken: credentials.sessionToken,
      StartTime: startTime,
      ExpiredTime: expiredTime,
    });
  }
});

function handleFile(e) {
  const file = e.target.files[0];
  if (!file) {
    console.warn('no file');
    return;
  }
  const name = file.name;
  cos.putObject(
    {
      Bucket: 'kimi-1251502833',
      Region: 'ap-beijing',
      Key: `cos-test/${name}`,
      Body: file,
      onProgress: function(progressData) {
        console.log('percentage', JSON.stringify(progressData));
      }
    },
    function(err, data) {
      if (err) {
        console.error('upload fail', err);
      } else {
        console.info('upload success', data);
      }
    },
  );
}

const input = document.createElement('input');
input.type = 'file';
input.onchange = handleFile;
document.body.appendChild(input);