## Binary memory segments
* `Sc` — scalar
* `Vec` — vector
* `&` — pointer
* `FP` — fat pointer

### 1. Single values
<table>
  <thead>
    <tr>
      <th></th>
      <th>text</th>
      <th>data</th>
      <th>rodata</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>const <code>Sc</code></td>
      <td>bytes</td>
      <td></td>
      <td></td>
    </tr>
    <tr>
      <td>const <code>Vec</code></td>
      <td><strong>&amp;</strong> —&gt;</td>
      <td></td>
      <td>bytes</td>
    </tr>
    <tr>
      <td>static <code>Sc</code></td>
      <td></td>
      <td></td>
      <td>bytes</td>
    </tr>
    <tr>
      <td>static <code>Vec</code></td>
      <td></td>
      <td></td>
      <td><strong>FP</strong> —&gt; bytes</td>
    </tr>
    <tr>
      <td>mut static <code>Sc</code></td>
      <td></td>
      <td>mut bytes</td>
      <td></td>
    </tr>
    <tr>
      <td>mut static <code>Vec</code></td>
      <td></td>
      <td>mut <strong>FP</strong> —&gt;</td>
      <td>bytes</td>
    </tr>
  </tbody>
</table>

### 2. Arrays
<table>
  <thead>
    <tr>
      <th></th>
      <th>text</th>
      <th>data</th>
      <th>rodata</th>
    </tr>
  </thead>
  <tbody>
      <td>const <code>Sc</code></td>
      <td>bytes</td>
      <td></td>
      <td></td>
    </tr>
    <tr>
      <td>const <code>Vec</code></td>
      <td><strong>FPₙ</strong> —&gt;</td>
      <td></td>
      <td>bytes</td>
    </tr>
    <tr>
      <td>const <strong>&amp;</strong> <code>Sc</code></td>
      <td><strong>FP</strong> —&gt;</td>
      <td></td>
      <td>bytes</td>
    </tr>
    <tr>
      <td>const <strong>&amp;</strong> <code>Vec</code></td>
      <td><strong>FP</strong> —&gt;</td>
      <td></td>
      <td><strong>FPₙ</strong> —&gt; bytes</td>
    </tr>
    <tr><td colspan="4" style="text-align: center;">⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯</td></tr>
    <tr>
      <td>static <code>Sc</code></td>
      <td></td>
      <td></td>
      <td>bytes</td>
    </tr>
    <tr>
      <td>static <code>Vec</code></td>
      <td></td>
      <td></td>
      <td><strong>FPₙ</strong> —&gt; bytes</td>
    </tr>
    <tr>
      <td>static <strong>&amp;</strong> <code>Sc</code></td>
      <td></td>
      <td></td>
      <td><strong>FP</strong> —&gt; bytes</td>
    </tr>
    <tr>
      <td>static <strong>&amp;</strong> <code>Vec</code></td>
      <td></td>
      <td></td>
      <td><strong>FP</strong> —&gt; <strong>FPₙ</strong> —&gt; bytes</td>
    </tr>
    <tr><td colspan="4" style="text-align: center;">⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯</td></tr>
    <tr>
      <td>mut static <code>Sc</code></td>
      <td></td>
      <td>mut bytes</td>
      <td></td>
    </tr>
    <tr>
      <td>mut static <code>Vec</code></td>
      <td></td>
      <td>mut <strong>FPₙ</strong> —&gt;</td>
      <td>bytes</td>
    </tr>
    <tr>
      <td>mut static <strong>&amp;</strong> <code>Sc</code></td>
      <td></td>
      <td>mut <strong>FP</strong> —&gt;</td>
      <td>bytes</td>
    </tr>
    <tr>
      <td>mut static <strong>&amp;</strong> <code>Vec</code></td>
      <td></td>
      <td>mut <strong>FP</strong> —&gt;</td>
      <td><strong>FPₙ</strong> —&gt; bytes</td>
    </tr>
    <tr>
      <td>mut static <strong>&amp;mut</strong> <code>Sc</code></td>
      <td></td>
      <td>mut <strong>FP</strong> —&gt; mut bytes</td>
      <td></td>
    </tr>
    <tr>
      <td>mut static <strong>&amp;mut</strong> <code>Vec</code></td>
      <td></td>
      <td>mut <strong>FP</strong> —&gt; mut <strong>FPₙ</strong> —&gt;</td>
      <td>bytes</td>
    </tr>
  </tbody>
</table>